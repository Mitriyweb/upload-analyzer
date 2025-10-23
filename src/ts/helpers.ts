/**
 * Helper functions for upload-analyzer
 * Runtime implementations for type guards and parsers
 */

import type { 
  FileInfo,
  FileAnalysis, 
  PEAnalysis, 
  MSIAnalysis,
  ELFAnalysis,
  MachOAnalysis,
  AnalysisError
} from './types';

// ========== Type Guards ==========

export function isPEAnalysis(analysis: FileAnalysis): analysis is PEAnalysis {
  return 'Format' in analysis && analysis.Format === 'PE';
}

export function isMSIAnalysis(analysis: FileAnalysis): analysis is MSIAnalysis {
  return 'Format' in analysis && analysis.Format === 'MSI';
}

export function isELFAnalysis(analysis: FileAnalysis): analysis is ELFAnalysis {
  return 'Format' in analysis && analysis.Format === 'ELF';
}

export function isMachOAnalysis(analysis: FileAnalysis): analysis is MachOAnalysis {
  return 'Format' in analysis && analysis.Format === 'Mach-O';
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
