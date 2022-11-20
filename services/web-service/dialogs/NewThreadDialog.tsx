import { Button, Dialog, DialogActions, DialogContent, DialogTitle } from '@mui/material'
import { useDefineDialog } from '../hooks'
import { DialogInfo } from '../components/DialogProvider'

export const NewThreadDialogInfo: DialogInfo<undefined>  = { name: 'NewThreadDialog' }

export default function NewThreadDialog() {
  const { props } = useDefineDialog(NewThreadDialogInfo)

  return (
    <Dialog {...props}>
      <DialogTitle>
        New Thread
      </DialogTitle>

      <DialogContent>

      </DialogContent>

      <DialogActions>
        <Button color="error" onClick={props.onClose}>
          Cancel
        </Button>
      </DialogActions>
    </Dialog>
  )
}
