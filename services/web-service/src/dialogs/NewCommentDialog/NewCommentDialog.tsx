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
import { createCommentQuery, CreateCommentResponse } from './api'

export interface NewCommentDialogProps {
  open: boolean
  onClose: (open: boolean) => void
  threadId: string
}

function NewCommentDialog({ open, onClose, threadId }: NewCommentDialogProps) {
  const [ createThread ] = useMutation<CreateCommentResponse>(createCommentQuery)
  const [ error, setError ] = useState<string | null>('')

  const formik = useFormik({
    initialValues:    { text: '' },
    validationSchema: Yup.object({
      text: Yup.string()
        .trim()
        .min(8, 'Must be at least 8 characters')
        .max(2048, 'Must be at most 2048 characters')
        .required('Required')
    }),
    onSubmit: async (values, { setFieldError }) => {
      const response = await createThread({
        variables: {
          text:     values.text,
          threadId: threadId
        }
      })

      if (response.data) {
        const result = response.data.createComment
        switch (result.__typename) {
        case 'Comment':
          handleClose(undefined, undefined, true)
          setError(null)
          break
        case 'Unauthenticated':
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

  const sharedProps = (name: 'text') => ({
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
          New Comment
        </DialogTitle>

        <DialogContent>
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

export default memo(NewCommentDialog)
