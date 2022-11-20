import { useContext } from 'react'
import { dialogContext } from '../components/DialogProvider'

export default function useDialogs() {
  const context = useContext(dialogContext)
  if (!context) {
    throw new Error('DialogProvider missing')
  }

  return { showDialog: context.showDialog }
}
