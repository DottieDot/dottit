import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  FormHelperText,
  LinearProgress,
  TextField
} from '@mui/material'
import { memo, useCallback, useState } from 'react'
import * as Yup from 'yup'
import { useFormik } from 'formik'
import { useMutation } from '@apollo/client'
import { createThreadQuery, CreateThreadResponse } from './api'
import { useSearchParams } from 'react-router-dom'

export interface NewThreadDialogProps {
  open: boolean
  onClose: (open: boolean) => void
  board: string
}

function NewThreadDialog({ open, onClose, board }: NewThreadDialogProps) {
  const [ createThread ] = useMutation<CreateThreadResponse>(createThreadQuery)
  const [ error, setError ] = useState<string | null>('')
  const[ , setParams ] = useSearchParams()

  const formik = useFormik({
    initialValues: {
      title: '',
      text:  ''
    },
    validationSchema: Yup.object({
      title: Yup.string()
        .trim()
        .min(8, 'Must be at least 8 characters')
        .max(128, 'Must be at most 128 characters')
        .required('Required'),
      text: Yup.string()
        .trim()
        .min(8, 'Must be at least 8 characters')
        .max(2048, 'Must be at most 2048 characters')
        .required('Required')
    }),
    onSubmit: async (values, { setFieldError }) => {
      const response = await createThread({
        variables: {
          title:   values.title,
          text:    values.text,
          boardId: board
        }
      })

      if (response.data) {
        const result = response.data.createThread
        switch (result.__typename) {
        case 'Thread':
          handleClose(undefined, undefined, true)
          setError(null)
          setParams({ thread: result.id })
          break
        case 'Unauthorized':
          setError(result.message)
          break
        case 'ValidationError':
          setError(result.message)
          for (const { field, errors: [ error ] } of result.errors) {
            setFieldError(field, error)
          }
          break
        }
      }
    },
    validateOnMount: true
  })

  const sharedProps = (name: 'title' | 'text') => ({
    fullWidth:  true,
    required:   true,
    name:       name,
    onChange:   formik.handleChange,
    onBlur:     formik.handleBlur,
    value:      formik.values[name],
    disabled:   formik.isSubmitting,
    error:      !!(formik.touched[name] && formik.errors[name]),
    helperText: formik.touched[name] && formik.errors[name] || ' '
  })

  const handleClose = useCallback((e?: unknown, unk?: unknown, force = false) => {
    if (!formik.isSubmitting || force) {
      formik.resetForm()
      onClose(false)
      setError(null)
    }
  }, [ onClose, formik ])

  return (
    <Dialog onClose={handleClose} open={open} fullWidth>
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
            {...sharedProps('title')}
          />

          <TextField
            label="Text"
            margin="normal"
            placeholder="Text..."
            rows={8}
            type="text"
            multiline
            {...sharedProps('text')}
          />

          <FormHelperText sx={{ mb: 1 }} error>
            {error ?? ' '}
          </FormHelperText>
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
