import type { FileInfo, FileAnalysis, PEAnalysis, MSIAnalysis, ELFAnalysis, MachOAnalysis, AnalysisError } from './types';
export declare function isPEAnalysis(analysis: FileAnalysis): analysis is PEAnalysis;
export declare function isMSIAnalysis(analysis: FileAnalysis): analysis is MSIAnalysis;
export declare function isELFAnalysis(analysis: FileAnalysis): analysis is ELFAnalysis;
export declare function isMachOAnalysis(analysis: FileAnalysis): analysis is MachOAnalysis;
export declare function isAnalysisError(analysis: FileAnalysis): analysis is AnalysisError;
export declare function parseFileInfo(jsonString: string): FileInfo;
export declare function parseAnalysis(jsonString: string): FileAnalysis;
//# sourceMappingURL=helpers.d.ts.map