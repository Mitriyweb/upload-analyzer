/**
 * Helper functions for upload-analyzer
 * Runtime implementations for type guards and parsers
 */

import type {
  FileInfo,
  FileAnalysis,
  PEAnalysis,
  MSIAnalysis,
  DMGAnalysis,
  DEBAnalysis,
  RPMAnalysis,
  AnalysisError
} from './types';

// ========== Type Guards ==========

export function isPEAnalysis(analysis: FileAnalysis): analysis is PEAnalysis {
  return 'Format' in analysis && analysis.Format === 'PE';
}

export function isMSIAnalysis(analysis: FileAnalysis): analysis is MSIAnalysis {
  return 'Format' in analysis && analysis.Format === 'MSI';
}

export function isDMGAnalysis(analysis: FileAnalysis): analysis is DMGAnalysis {
  return 'Format' in analysis && analysis.Format === 'DMG';
}

export function isDEBAnalysis(analysis: FileAnalysis): analysis is DEBAnalysis {
  return 'Format' in analysis && analysis.Format === 'DEB';
}

export function isRPMAnalysis(analysis: FileAnalysis): analysis is RPMAnalysis {
  return 'Format' in analysis && analysis.Format === 'RPM';
}

export function isAnalysisError(analysis: FileAnalysis): analysis is AnalysisError {
  return 'error' in analysis;
}

// ========== Parsers ==========

export function parseFileInfo(jsonString: string): FileInfo {
  return JSON.parse(jsonString) as FileInfo;
}

export function parseAnalysis(jsonString: string): FileAnalysis {
  return JSON.parse(jsonString) as FileAnalysis;
}
