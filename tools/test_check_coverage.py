"""Offline tests proving each coverage-gate check can fail.

A gate check that cannot fail for the artifact it claims to verify makes
every "tested" row unfalsifiable. Each test here fabricates the artifact the
check exists to reject and asserts the error fires — and that the legitimate
form still passes with nothing else firing. The real checked-in state is
asserted clean once, which is the property that gives the rest meaning.
"""

import unittest

import check_coverage

SPEC = b'{"paths": {}, "definitions": []}'
UNRELATED = (
    'pinned OpenAPI checksum changed',
    'endpoint drift changed',
    'definition drift changed',
)


def has(errors, fragment):
    return any(fragment in error for error in errors)


def unrelated(errors):
    """Errors every synthetic stub produces regardless of the check under test."""
    return [error for error in errors if not any(mark in error for mark in UNRELATED)]


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


class RealStateIsClean(unittest.TestCase):
    def test_the_checked_in_tree_passes_with_no_errors(self):
        self.assertEqual(check_coverage.check(*check_coverage.real_state()), [])


class DefinitionImplementation(unittest.TestCase):
    def test_string_alias_in_any_spelling_is_not_an_implemented_definition(self):
        for target in ('String', 'std::string::String', "&'static str",
                       'serde_json::Value', '()', 'u8', 'bool', 'Vec<String>'):
            with self.subTest(target=target):
                errors = check_coverage.check(
                    ledger(definitions=[definition()]), SPEC,
                    f'pub type Widget = {target};\n', '')
                self.assertTrue(has(errors, 'missing public definition: Widget'))

    def test_laundered_reexport_is_not_an_implemented_definition(self):
        source = 'pub type WidgetAlias = String;\npub use crate::WidgetAlias as Widget;\n'
        errors = check_coverage.check(ledger(definitions=[definition()]), SPEC, source, '')
        self.assertTrue(has(errors, 'missing public definition: Widget'))

    def test_fieldless_struct_is_not_an_implemented_definition(self):
        for declaration in ('pub struct Widget;', 'pub struct Widget {}'):
            with self.subTest(declaration=declaration):
                errors = check_coverage.check(
                    ledger(definitions=[definition()]), SPEC, declaration + '\n', '')
                self.assertTrue(has(errors, 'missing public definition: Widget'))

    def test_typed_alias_and_validated_reexport_pass_with_nothing_else_firing(self):
        source = ('pub type Widget = rust_decimal::Decimal;\n'
                  'pub struct Gadget { pub value: i64 }\n'
                  'pub use crate::Gadget as Doodad;\n')
        tests = ('#[test]\nfn widget_contract() {\n'
                 '    decode_fixture::<oanda_client::models::Widget>(json!("1.5"));\n'
                 '    decode_fixture::<oanda_client::models::Doodad>(json!({"value": 1}));\n'
                 '}\n')
        errors = check_coverage.check(
            ledger(definitions=[definition(),
                                definition(name='Doodad', test='widget_contract')]),
            SPEC, source, tests)
        self.assertEqual(unrelated(errors), [])

    def test_struct_enum_and_tuple_struct_carrying_types_pass(self):
        source = ('pub struct Widget { pub value: i64 }\n'
                  'pub enum Choice { A, Unknown(String) }\n'
                  'pub struct Stamp(DateTime<Utc>);\n')
        errors = check_coverage.check(
            ledger(definitions=[definition(),
                                definition(name='Choice', test='widget_contract'),
                                definition(name='Stamp', test='widget_contract')]),
            SPEC, source, '')
        self.assertEqual([e for e in unrelated(errors) if 'missing public definition' in e], [])


class DefinitionTested(unittest.TestCase):
    def test_a_mention_outside_the_marker_function_counts_for_nothing(self):
        tests = '''
#[test]
fn widget_contract() {
    // the Widget contract is checked elsewhere
}

#[test]
fn unrelated() {
    decode_fixture::<oanda_client::models::Widget>(json!({}));
}
'''
        errors = check_coverage.check(ledger(definitions=[definition()]), SPEC, '', tests)
        self.assertTrue(has(errors, 'test does not decode the definition: Widget'))

    def test_compile_time_bound_is_not_a_decode(self):
        tests = '''
#[test]
fn widget_contract() {
    typed_contract::<oanda_client::models::Widget>();
}
'''
        errors = check_coverage.check(ledger(definitions=[definition()]), SPEC, '', tests)
        self.assertTrue(has(errors, 'test does not decode the definition: Widget'))

    def test_decode_inside_the_marker_function_passes_cleanly(self):
        source = 'pub struct Widget { pub value: i64 }\n'
        tests = '''
#[test]
fn widget_contract() {
    decode_fixture::<oanda_client::models::Widget>(json!({"value": 1}));
}
'''
        errors = check_coverage.check(ledger(definitions=[definition()]), SPEC, source, tests)
        self.assertEqual(unrelated(errors), [])

    def test_marker_pointing_at_a_plain_helper_fails(self):
        source = 'pub struct Widget { pub value: i64 }\n'
        tests = '''
fn widget_contract() {
    decode_fixture::<oanda_client::models::Widget>(json!({"value": 1}));
}
'''
        errors = check_coverage.check(ledger(definitions=[definition()]), SPEC, source, tests)
        self.assertTrue(has(errors, 'missing definition test marker: Widget'))

    def test_marker_that_is_not_a_function_fails(self):
        tests = 'const widget_contract: &str = "widget_contract Widget";\n'
        errors = check_coverage.check(ledger(definitions=[definition()]), SPEC, '', tests)
        self.assertTrue(has(errors, 'missing definition test marker: Widget'))


class OperationTested(unittest.TestCase):
    SOURCE = '''
pub struct ListWidgetsResponse { pub value: i64 }
pub async fn list_widgets(&self)
    -> std::result::Result<crate::ApiResponse<ListWidgetsResponse>,
                           crate::OperationError<crate::GenericRejection>> { Ok(()) }
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

    def test_generic_rejection_must_appear_in_the_method_signature(self):
        source = self.SOURCE.replace('crate::OperationError<crate::GenericRejection>',
                                     'crate::OperationError<OtherRejection>')
        errors = check_coverage.check(
            ledger(operations=[operation()]), SPEC, source, self.TESTS)
        self.assertTrue(has(errors, 'generic rejection not typed in method'))


class OperationClassification(unittest.TestCase):
    def test_invalid_kind_fails(self):
        errors = check_coverage.check(ledger(operations=[operation(kind='banana')]), SPEC, '', '')
        self.assertTrue(has(errors, 'invalid kind'))
        self.assertTrue(has(errors, "'banana'"))

    def test_kind_disagreeing_with_the_path_fails(self):
        errors = check_coverage.check(
            ledger(operations=[operation(path='/v3/widgets/stream', kind='query',
                                         rejection='generic')]), SPEC, '', '')
        self.assertTrue(has(errors, 'disagrees with path'))

    def test_invalid_rejection_value_fails(self):
        errors = check_coverage.check(
            ledger(operations=[operation(rejection='invented')]), SPEC, '', '')
        self.assertTrue(has(errors, 'records no rejection decision'))


class StreamContract(unittest.TestCase):
    def test_any_stream_path_needs_a_typed_stream_event(self):
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

    def test_untyped_stream_element_fails(self):
        for element in ('String', 'serde_json::Value', 'Box<dyn std::error::Error>'):
            with self.subTest(element=element):
                source = (f'pub async fn stream_widget_events(&self) '
                          f'-> Result<HttpStream<{element}>> {{ todo!() }}\n')
                errors = check_coverage.check(
                    ledger(operations=[operation(
                        path='/v3/widgets/stream', public_method='stream_widget_events',
                        kind='stream', rejection='stream')]),
                    SPEC, source, '')
                self.assertTrue(has(errors, 'missing typed stream contract'))

    def test_a_comment_cannot_satisfy_the_stream_contract(self):
        source = ('pub async fn stream_widget_events(&self) '
                  '-> Result<()> /* HttpStream<WidgetStreamEvent> */ { todo!() }\n')
        errors = check_coverage.check(
            ledger(operations=[operation(
                path='/v3/widgets/stream', public_method='stream_widget_events',
                kind='stream', rejection='stream')]),
            SPEC, source, '')
        self.assertTrue(has(errors, 'missing typed stream contract'))

    def test_typed_stream_event_enum_passes_cleanly(self):
        source = ('pub enum WidgetStreamEvent { Beat }\n'
                  'pub async fn stream_widget_events(&self) '
                  '-> Result<HttpStream<WidgetStreamEvent>> { todo!() }\n')
        tests = ('#[test]\nfn widgets_fixture() {\n'
                 '    let _ = client.stream_widget_events();\n'
                 '}\n')
        errors = check_coverage.check(
            ledger(operations=[operation(
                path='/v3/widgets/stream', public_method='stream_widget_events',
                kind='stream', rejection='stream')]),
            SPEC, source, tests)
        self.assertEqual(unrelated(errors), [])


if __name__ == '__main__':
    unittest.main()
