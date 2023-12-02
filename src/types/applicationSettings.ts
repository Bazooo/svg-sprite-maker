export type TransparencyGridColor = [color: string, opacity: number]

export interface ApplicationSettings {
    // Generic
    autoSaveEnabled: boolean
    editorPath?: string
    darkMode: boolean

    // Transparency Grid
    transparencyGridColor1?: TransparencyGridColor
    transparencyGridColor2?: TransparencyGridColor
}
