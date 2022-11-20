import { createContext, ReactElement, useCallback, useMemo, useState } from 'react'

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export interface DialogInfo<TParams> {
  name: string
}

export interface DialogProviderProps {
  children: ReactElement
}

type DefineDialogReturnType<TParams> = {
  props: {
    open: false,
    onClose: () => void
  }
} | {
  props: {
    open: true,
    onClose: () => void
  },
  params: TParams
}

type ShowDialogType = <TParams>(dialogInfo: DialogInfo<TParams>, props: TParams) => void
type DefineDialogType = <TParams>(dialogInfo: DialogInfo<TParams>, onClose?: () => void) => DefineDialogReturnType<TParams>
type RemoveDialogType = (name: string) => void

export interface DialogContext {
  showDialog: ShowDialogType,
  defineDialog: DefineDialogType,
  removeDialog: RemoveDialogType
}

interface DialogState {
  open: boolean,
  onClose: () => void,
  params?: unknown
}

export const dialogContext = createContext<DialogContext|null>(null)

type State = {[name: string]: DialogState};

export default function DialogProvider({ children }: DialogProviderProps) {
  const [ dialogs, setDialogs ] = useState<State>({})
  const knownDialogs = useMemo(() => new Set(Object.keys(dialogs)), [ dialogs ])

  const showDialog = useCallback<ShowDialogType>((dialogInfo, params) => {
    setDialogs(dialogs => ({
      ...dialogs,
      [dialogInfo.name]: {
        ...dialogs[dialogInfo.name],
        open: true,
        params
      }
    }))
  }, [])

  const removeDialog = useCallback<RemoveDialogType>((name) => {
    setDialogs(dialogs => {
      return Object.keys(dialogs).reduce((acc: State, key: string) => {
        if (key !== name) {
          acc[key] = dialogs[key]
        }
        return acc
      }, {})
    })
  }, [])

  const defineDialog = useCallback<DefineDialogType>((dialogInfo, onClose) => {
    if (!knownDialogs.has(dialogInfo.name)) {
      setDialogs(dialogs => ({
        ...dialogs,
        [dialogInfo.name]: {
          onClose: () => {
            if (onClose) onClose()
            setDialogs(dialogs => ({
              ...dialogs,
              [dialogInfo.name]: {
                ...dialogs[dialogInfo.name],
                open:   false,
                params: undefined
              }
            }))
          },
          open:   false,
          params: undefined
        }
      }))
    }

    const dialog = dialogs[dialogInfo.name]

    if (dialog.open) {
      return {
        props: {
          open:    dialog.open,
          onClose: dialog.onClose
        },
        params: dialog.params as never
      }
    }
    else {
      return {
        props: {
          open:    dialog.open,
          onClose: dialog.onClose
        }
      }
    }
  }, [ knownDialogs, dialogs ])

  const value = useMemo<DialogContext>(() => ({
    showDialog,
    defineDialog,
    removeDialog
  }), [ removeDialog, defineDialog, showDialog ])

  return (
    <dialogContext.Provider value={value}>
      { children }
    </dialogContext.Provider>
  )
}
