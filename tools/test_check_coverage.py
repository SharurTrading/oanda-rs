"""Offline tests proving each coverage-gate check can fail.

A gate check that cannot fail for the artifact it claims to verify makes
every "tested" row unfalsifiable. Each test here fabricates the artifact the
check exists to reject and asserts the error fires — and that the legitimate
form still passes.
"""

import unittest

import check_coverage

SPEC = b'{"paths": {}, "definitions": []}'


def operation(**overrides):
    entry = {
        'method': 'GET', 'path': '/v3/widgets', 'public_method': 'list_widgets',
        'kind': 'query', 'rejection': 'generic', 'status': 'tested',
        'test': 'widgets_fixture',
    }
    entry.update(overrides)
    return entry


def definition(**overrides):
    entry = {'name': 'Widget', 'status': 'tested',
             'test': 'widget_contract'}
    entry.update(overrides)
    return entry


def ledger(operations=(), definitions=()):
    return {'operations': list(operations), 'definitions': list(definitions)}


def has(errors, fragment):
    return any(fragment in error for error in errors)


class DefinitionImplementation(unittest.TestCase):
    def test_string_alias_is_not_an_implemented_definition(self):
        source = 'pub type Widget = String;\n'
        errors = check_coverage.check(ledger(definitions=[definition()]), SPEC, source, '')
        self.assertTrue(has(errors, 'missing public definition: Widget'))

    def test_typed_alias_is_an_implemented_definition(self):
        source = 'pub type Widget = rust_decimal::Decimal;\n'
        errors = check_coverage.check(ledger(definitions=[definition()]), SPEC, source, '')
        self.assertFalse(has(errors, 'missing public definition: Widget'))

    def test_struct_and_id_type_are_implemented(self):
        source = 'pub struct Widget { pub value: i64 }\nid_type!(Gadget);\n'
        errors = check_coverage.check(
            ledger(definitions=[definition(),
                                definition(name='Gadget', test='widget_contract')]),
            SPEC, source, '')
        self.assertEqual([e for e in errors if 'missing public definition' in e], [])


class DefinitionTested(unittest.TestCase):
    def test_a_mention_outside_the_marker_function_counts_for_nothing(self):
        tests = '''
#[test]
fn widget_contract() {
    // the Widget contract is checked elsewhere
}

#[test]
fn unrelated() {
    typed_contract::<oanda_client::models::Widget>();
}
'''
        errors = check_coverage.check(ledger(definitions=[definition()]), SPEC, '', tests)
        self.assertTrue(has(errors, 'test does not use typed definition: Widget'))

    def test_usage_inside_the_marker_function_passes(self):
        tests = '''
#[test]
fn widget_contract() {
    typed_contract::<oanda_client::models::Widget>();
}
'''
        errors = check_coverage.check(ledger(definitions=[definition()]), SPEC, '', tests)
        self.assertFalse(has(errors, 'test does not use typed definition: Widget'))

    def test_marker_that_is_not_a_function_fails(self):
        tests = 'const widget_contract: &str = "widget_contract Widget";\n'
        errors = check_coverage.check(ledger(definitions=[definition()]), SPEC, '', tests)
        self.assertTrue(has(errors, 'missing definition test marker: Widget'))


class OperationTested(unittest.TestCase):
    SOURCE = '''
pub async fn list_widgets(&self) -> Result<()> { Ok(()) }
pub struct ListWidgetsResponse { pub value: i64 }
'''
    TESTS = '''
#[test]
fn widgets_fixture() {
    let _ = client.list_widgets();
}
'''

    def test_call_outside_the_marker_function_fails(self):
        tests = '''
#[test]
fn widgets_fixture() {
    assert!(true);
}

#[test]
fn other() {
    let _ = client.list_widgets();
}
'''
        errors = check_coverage.check(
            ledger(operations=[operation()]), SPEC, self.SOURCE, tests)
        self.assertTrue(has(errors, 'test does not call public method'))

    def test_missing_marker_function_fails(self):
        errors = check_coverage.check(
            ledger(operations=[operation(test='ghost')]), SPEC, self.SOURCE, self.TESTS)
        self.assertTrue(has(errors, 'missing test marker'))


class StreamContract(unittest.TestCase):
    def test_any_stream_operation_needs_a_typed_stream_contract(self):
        # Not one of the two historically hardcoded method names: the check
        # itself must be general.
        plain = '''
pub async fn stream_widget_events(&self) -> Result<()> { Ok(()) }
'''
        errors = check_coverage.check(
            ledger(operations=[operation(
                path='/v3/widgets/stream', public_method='stream_widget_events',
                kind='stream', rejection='stream')]),
            SPEC, plain, '')
        self.assertTrue(has(errors, 'missing typed stream contract'))

    def test_http_stream_return_satisfies_the_contract(self):
        typed = '''
pub async fn stream_widget_events(&self)
    -> Result<HttpStream<WidgetStreamEvent>> { todo!() }
'''
        errors = check_coverage.check(
            ledger(operations=[operation(
                path='/v3/widgets/stream', public_method='stream_widget_events',
                kind='stream', rejection='stream')]),
            SPEC, typed, '')
        self.assertEqual([e for e in errors if 'stream contract' in e], [])


if __name__ == '__main__':
    unittest.main()
