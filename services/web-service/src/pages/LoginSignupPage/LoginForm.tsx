import { FormHelperText, Link, TextField, Typography } from '@mui/material'
import { Link as RouterLink } from 'react-router-dom'
import { useFormik } from 'formik'
import * as Yup from 'yup'
import { LoadingButton } from '@mui/lab'
import { useMutation } from '@apollo/client'
import { loginQuery, LoginResponse } from './api'
import { useState } from 'react'
import { useUser } from '../../hooks'

export default function LoginForm() {
  const [ login ] = useMutation<LoginResponse>(loginQuery)
  const [ error, setError ] = useState<string| null>(null)
  const { setLoggedIn } = useUser()
  const formik = useFormik({
    initialValues: {
      username: '',
      password: ''
    },
    validationSchema: Yup.object({
      username: Yup.string().trim().required(),
      password: Yup.string().required()
    }),
    onSubmit: async (values) => {
      const response = await login({
        variables: {
          username: values.username,
          password: values.password
        }
      })
      if (response.data) {
        const result = response.data.loginUser
        switch (result.__typename) {
        case 'AuthenticatedUser':
          setLoggedIn(result.apiToken, result.user)
          break
        case 'AlreadyLoggedIn':
          setError(result.message)
          break
        case 'LoginFailed':
          setError(result.message)
          break
        }
      }
    },
    validateOnMount: true
  })

  const sharedProps = (name: 'username' | 'password') => ({
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
    <form onSubmit={formik.handleSubmit}>
      <Typography variant="h5" gutterBottom>
        Login
      </Typography>

      <TextField
        label="Username"
        margin="normal"
        type="text"
        variant="outlined"
        {...sharedProps('username')}
      />

      <TextField
        label="Password"
        margin="normal"
        type="password"
        variant="outlined"
        {...sharedProps('password')}
      />

      <LoadingButton
        disabled={formik.isSubmitting || !formik.isValid}
        loading={formik.isSubmitting}
        sx={{
          mt: 2,
          mb: 1
        }}
        type="submit"
        variant="contained"
        fullWidth
      >
        Login
      </LoadingButton>

      <FormHelperText sx={{ mb: 1 }} error>
        {error ?? ' '}
      </FormHelperText>

      <Link component={RouterLink} to="/auth/signup">
        Don&apos;t have an account? Create one instead.
      </Link>
    </form>
  )
}
