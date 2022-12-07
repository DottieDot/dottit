import { Button, Dialog, DialogActions, DialogContent, DialogTitle, FormHelperText, TextField } from '@mui/material'
import { useCallback, useState } from 'react'
import { useFormik } from 'formik'
import * as Yup from 'yup'
import { useMutation } from '@apollo/client'
import { createBoardMutation, CreateBoardResponse } from './api'
import { LoadingButton } from '@mui/lab'
import { useNavigate } from 'react-router'

interface NewBoardDialogProps {
  open: boolean
  onClose: (open: boolean) => void
}

export default function NewBoardDialog({ open, onClose }: NewBoardDialogProps) {
  const [ createBoard ] = useMutation<CreateBoardResponse>(createBoardMutation)
  const [ error, setError ] = useState<string|null>(null)
  const navigate = useNavigate()

  const formik = useFormik({
    initialValues:    { name: '' },
    validationSchema: Yup.object({
      name: Yup.string()
        .trim()
        .min(4, 'must be at least 4 characters long.')
        .max(24, 'must be 24 characters at most.')
        .required()
    }),
    onSubmit: async (values, { setFieldError }) => {
      const response = await createBoard({ variables: { name: values.name }})

      if (response.data) {
        const result = response.data.createBoard
        switch (result.__typename) {
        case 'Board':
          navigate(`/b/${result.name}`)
          break
        case 'Unauthenticated':
          setError(result.message)
          break
        case 'ValidationError':
          setError(result.message)
          result.errors.forEach(({ field, errors: [ error ] }) => {
            setFieldError(field, error)
          })
          break
        }
      }
    },
    validateOnMount: true
  })

  const handleClose = useCallback(() => {
    if (!formik.isSubmitting) {
      onClose(false)
      formik.resetForm()
    }
  }, [ formik, onClose ])

  const sharedProps = (name: 'name') => ({
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

  return (
    <Dialog
      maxWidth="sm"
      onClose={handleClose}
      open={open}
      fullWidth
    >
      <form onSubmit={formik.handleSubmit}>
        <DialogTitle>
          New Board
        </DialogTitle>

        <DialogContent>
          <TextField
            label="Name"
            margin="normal"
            type="text"
            variant="outlined"
            {...sharedProps('name')}
          />

          <FormHelperText sx={{ mb: 1 }} error>
            {error ?? ' '}
          </FormHelperText>
        </DialogContent>

        <DialogActions>
          <Button color="error" disabled={formik.isSubmitting} onClick={handleClose}>
            Cancel
          </Button>

          <LoadingButton disabled={!formik.isValid || formik.isSubmitting} loading={formik.isSubmitting} type="submit">
            Create
          </LoadingButton>
        </DialogActions>
      </form>
    </Dialog>
  )
}
