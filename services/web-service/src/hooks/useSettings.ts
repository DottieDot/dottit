import { useCallback, useContext } from 'react'
import { Settings, settingsContext } from '../components'

type SetSettings = (settings: Partial<Settings>) => void

export function useSettings(): [Settings, SetSettings] {
  const context = useContext(settingsContext)

  if (!context) {
    throw new Error('Invalid settings context state')
  }

  const { settings, setSettings } = context

  const setter = useCallback<SetSettings>((settings) => {
    setSettings((currentSettings) => ({
      ...currentSettings,
      ...settings
    }))
  }, [ setSettings ])

  return [ settings, setter ]
}
