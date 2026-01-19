import { test, describe } from 'node:test';
import assert from 'node:assert';
import { parseFileInfo, parseAnalysis } from '../../ts/helpers';

describe('Parsers', () => {
  test('parseFileInfo parses valid JSON', () => {
    const json = '{"Format": "PE", "Size": "1024"}';
    const info = parseFileInfo(json);
    assert.strictEqual(info.Format, 'PE');
    assert.strictEqual(info.Size, '1024');
  });

  test('parseAnalysis parses valid PE analysis JSON', () => {
    const json = '{"Format": "PE", "Architecture": "x64", "CompanyName": "Test Corp"}';
    const analysis = parseAnalysis(json);
    assert.strictEqual(analysis.Format, 'PE');
    if ('CompanyName' in analysis) {
      assert.strictEqual(analysis.CompanyName, 'Test Corp');
    } else {
      assert.fail('CompanyName should be in analysis');
    }
  });

  test('parseAnalysis parses error JSON', () => {
    const json = '{"error": "Failed to parse"}';
    const analysis = parseAnalysis(json);
    if ('error' in analysis) {
      assert.strictEqual(analysis.error, 'Failed to parse');
    } else {
      assert.fail('error should be in analysis');
    }
  });

  test('parseAnalysis throws on invalid JSON', () => {
    const json = 'invalid json';
    assert.throws(() => parseAnalysis(json), SyntaxError);
  });
});
