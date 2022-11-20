import { dialogContext, DialogInfo } from '../components/DialogProvider'
import { useContext, useEffect } from 'react'

export default function useDefineDialog<TParams>(info: DialogInfo<TParams>, onClose?: () => void) {
  const context = useContext(dialogContext)
  if (!context) {
    throw new Error('DialogProvider missing')
  }

  const { defineDialog, removeDialog } = context

  useEffect(() => {
    return () => {
      removeDialog(info.name)
    }
  }, [ info, removeDialog ])

  return defineDialog(info, onClose)
}
