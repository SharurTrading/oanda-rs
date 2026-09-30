"""Offline failure-path tests for the contract generators.

A generator must never substitute a plausible default for provider content:
every parse miss exits non-zero, writes nothing, and names the offending
endpoint or definition. These tests build synthetic saved pages and assert
both the exit and the untouched destination.
"""

import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

try:
    from lxml import html as _lxml_probe  # noqa: F401
    LXML_AVAILABLE = True
except ImportError:
    LXML_AVAILABLE = False

# The generators parse saved provider pages with lxml, which the deliberately
# dependency-free CI runner does not install. Their failure paths are exercised
# wherever the generators themselves run; everywhere else this suite skips
# visibly rather than erroring a gate it cannot execute.
if LXML_AVAILABLE:
    import generate_models
    import generate_api

DF_NAMES = ('account', 'instrument', 'order', 'trade', 'position',
            'transaction', 'pricing', 'pricing-common', 'primitives')
EP_CAPS = ('account', 'order', 'trade', 'position', 'transaction', 'pricing')

SCHEMA_PRE = 'json_schema'
PARAM_ROWS = '''
<tr><td>Authorization</td><td>header</td><td>string</td>
<td>The authorization bearer token [required]</td></tr>
<tr><td>accountID</td><td>path</td><td>accountID</td>
<td>Account Identifier [required]</td></tr>
'''
RESPONSE_SCHEMA = '{\n    # The accounts.\n    accounts : (array[Account]),\n}'
REJECTION_SCHEMA = '{\n    # The code.\n    errorCode : (string, required),\n}'


def df_page(body):
    return f'''<html><body>
<div class="endpoint_header"><span class="method">Widget</span>
<span class="definition">A documented widget.</span>
<a href="#collapse_definition_1"></a></div>
<div id="collapse_definition_1">{body}</div>
</body></html>'''


def ep_page(parameters=PARAM_ROWS, response_schema=RESPONSE_SCHEMA,
            rejection_schema=None, method='GET', path='/v3/accounts'):
    rejection = (f'<div id="collapse_1_400"><pre class="{SCHEMA_PRE}">{rejection_schema}</pre></div>'
                 if rejection_schema is not None else '')
    params = ''
    if parameters is not None:
        params = f'''<div id="collapse_1_parameters"><table class="parameter_table">{parameters}</table></div>'''
    return f'''<html><body>
<div class="endpoint_header method_{method.lower()}">
<span class="method">{method}</span>
<span class="path">{path}<p>Summary.</p></span>
{params}
<div id="collapse_1_200"><pre class="{SCHEMA_PRE}">{response_schema}</pre></div>
{rejection}
</div>
</body></html>'''


def blank_page():
    return '<html><body><p>no definitions</p></body></html>'


class GeneratorTestCase(unittest.TestCase):
    def scaffold(self, df_pages=None, ep_pages=None, with_rejection=False):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        base = Path(self.temporary.name)
        source = base / 'pages'
        source.mkdir()
        for name in DF_NAMES:
            content = (df_pages or {}).get(name, blank_page())
            (source / f'oanda-{name}-df.html').write_text(content, encoding='utf-8')
        for cap in EP_CAPS:
            content = (ep_pages or {}).get(cap, ep_page(
                rejection_schema=REJECTION_SCHEMA if with_rejection else None))
            (source / f'oanda-{cap}-ep.html').write_text(content, encoding='utf-8')
        destination = base / 'dest'
        destination.mkdir()
        return source, destination

    def assert_models_fail_untouched(self, source, destination, pattern):
        sentinel = destination / 'account.rs'
        sentinel.write_text('sentinel\n', encoding='utf-8')
        with patch('sys.stderr'):
            with self.assertRaises(SystemExit) as raised:
                generate_models.main(source, destination)
        self.assertTrue(bool(raised.exception.code), 'generator must exit non-zero')
        self.assertIn(pattern, str(raised.exception.code))
        self.assertEqual(sentinel.read_text(encoding='utf-8'), 'sentinel\n',
                         'a failed run must not touch destination files')

    def assert_api_fail_untouched(self, source, root, pattern):
        (root / 'src').mkdir(exist_ok=True)
        ledger = root / 'docs' / 'coverage.json'
        ledger.parent.mkdir(exist_ok=True)
        sentinel = root / 'src' / 'account.rs'
        sentinel.write_text('sentinel\n', encoding='utf-8')
        with patch('sys.stderr'):
            with self.assertRaises(SystemExit) as raised:
                generate_api.main(source, root)
        self.assertTrue(bool(raised.exception.code), 'generator must exit non-zero')
        self.assertIn(pattern, str(raised.exception.code))
        self.assertEqual(sentinel.read_text(encoding='utf-8'), 'sentinel\n',
                         'a failed run must not touch destination files')


@unittest.skipUnless(LXML_AVAILABLE, 'lxml is not installed; generator failure paths run where the generators run')
class GenerateModelsFailures(GeneratorTestCase):
    def test_unparsed_definition_refuses_a_string_alias(self):
        source, destination = self.scaffold({'account': df_page('<p>neither schema nor table</p>')})
        self.assert_models_fail_untouched(
            source, destination, 'account-df Widget')

    def test_unrecognized_schema_token_refuses_guessed_requiredness(self):
        schema = '{\n    # A field.\n    units : (DecimalNumber, requred),\n}'
        source, destination = self.scaffold({'account': df_page(
            f'<pre class="{SCHEMA_PRE}">{schema}</pre>')})
        self.assert_models_fail_untouched(
            source, destination, "Widget.units: unrecognized schema token 'requred'")

    def test_field_without_provider_description_fails(self):
        schema = '{\n    units : (DecimalNumber, required),\n}'
        source, destination = self.scaffold({'account': df_page(
            f'<pre class="{SCHEMA_PRE}">{schema}</pre>')})
        self.assert_models_fail_untouched(
            source, destination, 'Widget.units: field has no provider description')

    def test_enum_without_rows_fails(self):
        table = '<table><tr><th>Value</th><th>Description</th></tr></table>'
        source, destination = self.scaffold({'account': df_page(table)})
        self.assert_models_fail_untouched(
            source, destination, 'Widget: value table has no parseable rows')


@unittest.skipUnless(LXML_AVAILABLE, 'lxml is not installed; generator failure paths run where the generators run')
class GenerateApiFailures(GeneratorTestCase):
    def test_missing_success_response_refuses_permissive_struct(self):
        source, _ = self.scaffold(ep_pages={'account': ep_page(response_schema='')})
        base = Path(self.temporary.name)
        root = base / 'repo'
        root.mkdir()
        self.assert_api_fail_untouched(
            source, root, 'no success response schema')

    def test_parameter_table_without_required_marker_fails(self):
        rows = ('<tr><td>Authorization</td><td>header</td><td>string</td>'
                '<td>The authorization bearer token.</td></tr>')
        source, _ = self.scaffold(ep_pages={'account': ep_page(parameters=rows)})
        base = Path(self.temporary.name)
        root = base / 'repo'
        root.mkdir()
        self.assert_api_fail_untouched(
            source, root, 'no parameter is marked [required]')

    def test_endpoint_dto_without_provider_description_fails(self):
        import tempfile
        bare = '{\n    accounts : (array[Account]),\n}'
        source, _ = self.scaffold(ep_pages={'account': ep_page(response_schema=bare)})
        with tempfile.TemporaryDirectory() as directory:
            self.assert_api_fail_untouched(
                source, Path(directory), 'endpoint DTO field has no provider description')

    def test_partial_requiredness_drift_against_the_spec_fails(self):
        # Authorization keeps its marker, so the all-optional guard passes;
        # accountID alone loses [required], which only the independent pinned
        # spec can contradict.
        rows = ('<tr><td>Authorization</td><td>header</td><td>string</td>'
                '<td>The authorization bearer token [required]</td></tr>'
                '<tr><td>accountID</td><td>path</td><td>accountID</td>'
                '<td>Account Identifier</td></tr>')
        source, _ = self.scaffold(ep_pages={'account': ep_page(
            method='GET', path='/v3/accounts/{accountID}/orders', parameters=rows)})
        import tempfile
        with tempfile.TemporaryDirectory() as directory:
            self.assert_api_fail_untouched(
                source, Path(directory),
                'parameter accountID is optional on the website but required in the pinned spec')

    def test_missing_parameter_table_fails(self):
        source, _ = self.scaffold(ep_pages={'account': ep_page(parameters=None)})
        base = Path(self.temporary.name)
        root = base / 'repo'
        root.mkdir()
        self.assert_api_fail_untouched(
            source, root, 'no parameter table')


@unittest.skipUnless(LXML_AVAILABLE, 'lxml is not installed; generator failure paths run where the generators run')
class GenerateApiRejectionDecision(GeneratorTestCase):
    def test_undocumented_rejection_is_generic_not_invented(self):
        source, _ = self.scaffold()
        base = Path(self.temporary.name)
        root = base / 'repo'
        (root / 'src').mkdir(parents=True)
        (root / 'docs').mkdir(parents=True)
        generate_api.main(source, root)
        account = (root / 'src' / 'account.rs').read_text(encoding='utf-8')
        self.assertIn('OperationError<crate::GenericRejection>', account)
        self.assertNotIn('errorCode', account)
        ledger = (root / 'docs' / 'coverage.json').read_text(encoding='utf-8')
        self.assertIn('"rejection": "generic"', ledger)

    def test_documented_rejection_is_derived(self):
        source, _ = self.scaffold(with_rejection=True)
        base = Path(self.temporary.name)
        root = base / 'repo'
        (root / 'src').mkdir(parents=True)
        (root / 'docs').mkdir(parents=True)
        generate_api.main(source, root)
        account = (root / 'src' / 'account.rs').read_text(encoding='utf-8')
        self.assertIn('The code.', account)
        ledger = (root / 'docs' / 'coverage.json').read_text(encoding='utf-8')
        self.assertIn('"rejection": "endpoint"', ledger)


if __name__ == '__main__':
    unittest.main()
