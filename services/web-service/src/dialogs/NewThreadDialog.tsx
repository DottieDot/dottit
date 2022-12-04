import { Button, Dialog, DialogActions, DialogContent, DialogTitle, LinearProgress, TextField } from '@mui/material'
import { memo, useCallback } from 'react'
import * as Yup from 'yup'
import { useFormik } from 'formik'

export interface NewThreadDialogProps {
  open: boolean
  onClose: (open: boolean) => void
  board: string
}

function NewThreadDialog({ open, onClose }: NewThreadDialogProps) {
  const formik = useFormik({
    initialValues: {
      title: '',
      text:  ''
    },
    validationSchema: Yup.object({
      title: Yup.string()
        .trim()
        .max(128)
        .required('Required'),
      text: Yup.string()
        .trim()
        .max(2048)
        .required('Required')
    }),
    onSubmit: (values, { setSubmitting, resetForm }) => {
      setSubmitting(false)
      resetForm()
    },
    validateOnMount: true
  })

  const sharedProps = (name: 'title' | 'text') => ({
    fullWidth: true,
    required:  true,
    name:      name,
    onChange:  formik.handleChange,
    onBlur:    formik.handleBlur,
    value:     formik.values[name],
    disabled:  formik.isSubmitting,
    error:     !!(formik.touched[name] && formik.errors[name])
  })

  const handleClose = useCallback(() => {
    if (!formik.isSubmitting) {
      formik.resetForm()
      onClose(false)
    }
  }, [ onClose, formik ])

  return (
    <Dialog open={open} fullWidth>
      <form onSubmit={formik.handleSubmit}>
        {formik.isSubmitting && <LinearProgress />}

        <DialogTitle>
          New Thread
        </DialogTitle>

        <DialogContent>
          <TextField
            label="Title"
            margin="normal"
            placeholder="An interesting title."
            type="text"
            autoFocus
            {...sharedProps('title')}
          />

          <TextField
            label="Text"
            margin="normal"
            placeholder="Text..."
            rows={8}
            type="text"
            autoFocus
            multiline
            {...sharedProps('text')}
          />
        </DialogContent>

        <DialogActions>
          <Button
            color="error"
            disabled={formik.isSubmitting}
            onClick={handleClose}
          >
            Cancel
          </Button>

          <Button
            color="primary"
            disabled={formik.isSubmitting || !formik.isValid}
            type="submit"
          >
            Post
          </Button>
        </DialogActions>
      </form>
    </Dialog>
  )
}

export default memo(NewThreadDialog)
