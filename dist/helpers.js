export function isPEAnalysis(analysis) {
    return 'Format' in analysis && analysis.Format === 'PE';
}
export function isMSIAnalysis(analysis) {
    return 'Format' in analysis && analysis.Format === 'MSI';
}
export function isELFAnalysis(analysis) {
    return 'Format' in analysis && analysis.Format === 'ELF';
}
export function isMachOAnalysis(analysis) {
    return 'Format' in analysis && analysis.Format === 'Mach-O';
}
export function isAnalysisError(analysis) {
    return 'error' in analysis;
}
export function parseFileInfo(jsonString) {
    return JSON.parse(jsonString);
}
export function parseAnalysis(jsonString) {
    return JSON.parse(jsonString);
}
//# sourceMappingURL=helpers.js.map