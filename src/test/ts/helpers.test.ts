import test from 'node:test';
import assert from 'node:assert';
import {
  isPEAnalysis,
  isMSIAnalysis,
  isDMGAnalysis,
  isDEBAnalysis,
  isRPMAnalysis,
  isAnalysisError,
  parseFileInfo,
  parseAnalysis
} from '../../ts/helpers';
import type { FileAnalysis } from '../../ts/types';

test('isPEAnalysis identifies PE format', () => {
  const analysis: FileAnalysis = { Format: 'PE', Architecture: 'x64' };
  assert.strictEqual(isPEAnalysis(analysis), true);
  assert.strictEqual(isMSIAnalysis(analysis), false);
});

test('isMSIAnalysis identifies MSI format', () => {
  const analysis: FileAnalysis = { Format: 'MSI', Architecture: 'Windows Installer Package' };
  assert.strictEqual(isMSIAnalysis(analysis), true);
  assert.strictEqual(isPEAnalysis(analysis), false);
});

test('isDMGAnalysis identifies DMG format', () => {
  const analysis: FileAnalysis = { Format: 'DMG', Architecture: 'macOS Disk Image' };
  assert.strictEqual(isDMGAnalysis(analysis), true);
  assert.strictEqual(isPEAnalysis(analysis), false);
});

test('isDEBAnalysis identifies DEB format', () => {
  const analysis: FileAnalysis = { Format: 'DEB' };
  assert.strictEqual(isDEBAnalysis(analysis), true);
  assert.strictEqual(isPEAnalysis(analysis), false);
});

test('isRPMAnalysis identifies RPM format', () => {
  const analysis: FileAnalysis = { Format: 'RPM' };
  assert.strictEqual(isRPMAnalysis(analysis), true);
  assert.strictEqual(isPEAnalysis(analysis), false);
});

test('isAnalysisError identifies error response', () => {
  const analysis: FileAnalysis = { error: 'Failed to parse' };
  assert.strictEqual(isAnalysisError(analysis), true);
  assert.strictEqual(isPEAnalysis(analysis as any), false);
});

test('parseFileInfo parses JSON string', () => {
  const json = '{"Format": "PE", "Size": "1024"}';
  const info = parseFileInfo(json);
  assert.strictEqual(info.Format, 'PE');
  assert.strictEqual(info.Size, '1024');
});

test('parseAnalysis parses JSON string', () => {
  const json = '{"Format": "PE", "Architecture": "x64"}';
  const analysis = parseAnalysis(json);
  assert.strictEqual(analysis.Format, 'PE');
  if ('Architecture' in analysis) {
    assert.strictEqual(analysis.Architecture, 'x64');
  }
});
