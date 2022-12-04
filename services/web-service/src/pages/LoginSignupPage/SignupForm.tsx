import { FormHelperText, Link, TextField, Typography } from '@mui/material'
import { useFormik } from 'formik'
import * as Yup from 'yup'
import { useMutation } from '@apollo/client'
import { createUserQuery, CreateUserResponse } from './api'
import { Link as RouterLink } from 'react-router-dom'
import { LoadingButton } from '@mui/lab'
import { useState } from 'react'
import { useUser } from '../../hooks'

export default function SignupForm() {
  const [ createUser ] = useMutation<CreateUserResponse>(createUserQuery)
  const [ error, setError ] = useState<string| null>(null)
  const { setLoggedIn } = useUser()
  const formik = useFormik({
    initialValues: {
      username:        '',
      password:        '',
      confirmPassword: ''
    },
    validationSchema: Yup.object({
      username: Yup.string()
        .min(4, 'Must at least be 4 characters')
        .max(24, 'Must be below 24 characters')
        .matches(/^[a-zA-Z0-9]+$/, 'Username must be alphanumerical')
        .trim()
        .required('Required'),
      password: Yup.string()
        .max(80, 'Must be below 80 characters')
        .min(8, 'Must be at least 8 characters long')
        .matches(/[a-z]/g, 'Must contain at least 1 lower case characer')
        .matches(/[A-Z]/g, 'Must contain at least 1 upper case characer')
        .matches(/[0-9]/g, 'Must contain at least 1 numeric characer')
        .required('Required'),
      confirmPassword: Yup.string()
        .oneOf([ Yup.ref('password') ], 'Passwords do not match')
        .required('Required')
    }),
    onSubmit: async (values, { setFieldError }) => {
      const response = await createUser({
        variables: {
          username: values.username,
          password: values.password
        }
      })

      if (response.data) {
        const result = response.data.createUser
        switch (result.__typename) {
        case 'AuthenticatedUser':
          setLoggedIn(result.apiToken, result.user)
          break
        case 'AlreadyLoggedIn':
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

  const sharedProps = (name: 'username' | 'password' | 'confirmPassword') => ({
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
        Signup
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

      <TextField
        label="Repeat Password"
        margin="normal"
        type="password"
        variant="outlined"
        {...sharedProps('confirmPassword')}
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
        Create Account
      </LoadingButton>

      <FormHelperText sx={{ mb: 1 }} error>
        {error ?? ' '}
      </FormHelperText>

      <Link component={RouterLink} to="/auth/login">
        Already have an account? Login instead.
      </Link>
    </form>
  )
}
