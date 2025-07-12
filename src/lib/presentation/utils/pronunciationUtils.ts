export interface PronunciationData {
  phonetic?: string;
  audio?: string;
}

/**
 * Parse pronunciation JSON string safely
 */
export function parsePronunciation(json: string | null | undefined): PronunciationData | null {
  try {
    return json ? JSON.parse(json) : null;
  } catch (error) {
    console.warn('Failed to parse pronunciation data:', error);
    return null;
  }
}

/**
 * Stringify pronunciation data safely
 */
export function stringifyPronunciation(data: PronunciationData): string {
  try {
    return JSON.stringify(data);
  } catch (error) {
    console.warn('Failed to stringify pronunciation data:', error);
    return JSON.stringify({});
  }
}

/**
 * Check if pronunciation data has audio
 */
export function hasAudio(pronunciation: string | null | undefined): boolean {
  const data = parsePronunciation(pronunciation);
  return Boolean(data?.audio);
}

/**
 * Get phonetic from pronunciation data
 */
export function getPhonetic(pronunciation: string | null | undefined): string {
  const data = parsePronunciation(pronunciation);
  return data?.phonetic || '';
}

/**
 * Get audio URL from pronunciation data
 */
export function getAudioUrl(pronunciation: string | null | undefined): string {
  const data = parsePronunciation(pronunciation);
  return data?.audio || '';
}
