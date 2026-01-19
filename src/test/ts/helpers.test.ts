import { test, describe } from 'node:test';
import assert from 'node:assert';
import {
  isPEAnalysis,
  isMSIAnalysis,
  isDMGAnalysis,
  isDEBAnalysis,
  isRPMAnalysis,
  isAnalysisError
} from '../../ts/helpers';
import type { FileAnalysis } from '../../ts/types';

describe('Type Guards', () => {
  test('isPEAnalysis identifies PE format', () => {
    const analysis = { Format: 'PE', Architecture: 'x64' } as FileAnalysis;
    assert.strictEqual(isPEAnalysis(analysis), true);
    assert.strictEqual(isMSIAnalysis(analysis), false);
  });

  test('isMSIAnalysis identifies MSI format', () => {
    const analysis = { Format: 'MSI', Architecture: 'Windows Installer Package' } as FileAnalysis;
    assert.strictEqual(isMSIAnalysis(analysis), true);
    assert.strictEqual(isPEAnalysis(analysis), false);
  });

  test('isDMGAnalysis identifies DMG format', () => {
    const analysis = { Format: 'DMG', Architecture: 'macOS Disk Image' } as FileAnalysis;
    assert.strictEqual(isDMGAnalysis(analysis), true);
    assert.strictEqual(isPEAnalysis(analysis), false);
  });

  test('isDEBAnalysis identifies DEB format', () => {
    const analysis = { Format: 'DEB' } as FileAnalysis;
    assert.strictEqual(isDEBAnalysis(analysis), true);
  });

  test('isRPMAnalysis identifies RPM format', () => {
    const analysis = { Format: 'RPM' } as FileAnalysis;
    assert.strictEqual(isRPMAnalysis(analysis), true);
  });

  test('isAnalysisError identifies errors', () => {
    const analysis = { error: 'File not found' } as FileAnalysis;
    assert.strictEqual(isAnalysisError(analysis), true);
    assert.strictEqual(isPEAnalysis(analysis), false);
  });
});
