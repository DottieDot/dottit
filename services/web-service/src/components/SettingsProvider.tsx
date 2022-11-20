import { createContext, Dispatch, memo, ReactNode, SetStateAction } from 'react'
import { useLocalStorage } from 'usehooks-ts'

export interface Settings {
  theme: 'system' | 'light' | 'dark'
}

export interface SettingsContext {
  settings: Settings,
  setSettings: Dispatch<SetStateAction<Settings>>
}

export interface SettingsProviderProps {
  defaultSettings: Settings,
  children: ReactNode
}

export const settingsContext = createContext<SettingsContext | null>(null)

function SettingsProvider({ defaultSettings, children }: SettingsProviderProps) {
  const [ settings, setSettings ] = useLocalStorage('settings', defaultSettings)

  return (
    <settingsContext.Provider
      value={{
        settings,
        setSettings
      }}
    >
      {children}
    </settingsContext.Provider>
  )
}

export default memo(SettingsProvider)
