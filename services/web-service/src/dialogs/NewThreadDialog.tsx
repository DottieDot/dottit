import { Button, Dialog, DialogActions, DialogContent, DialogTitle } from '@mui/material'
import { memo, useCallback } from 'react'

export interface NewThreadDialogProps {
  open: boolean
  onClose: (open: boolean) => void
}

function NewThreadDialog({ open, onClose }: NewThreadDialogProps) {
  const handleClose = useCallback(() => {
    onClose(false)
  }, [ onClose ])

  return (
    <Dialog open={open}>
      <DialogTitle>
        New Thread
      </DialogTitle>

      <DialogContent>

      </DialogContent>

      <DialogActions>
        <Button color="error" onClick={handleClose}>
          Cancel
        </Button>
      </DialogActions>
    </Dialog>
  )
}

export default memo(NewThreadDialog)
