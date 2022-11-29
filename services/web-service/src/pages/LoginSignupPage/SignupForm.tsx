import { Button, TextField, Typography } from '@mui/material'
import { useFormik } from 'formik'
import * as Yup from 'yup'
import { useMutation } from '@apollo/client'
import { createUserQuery, CreateUserResponse } from './api'

export default function SignupForm() {
  const [ createUser ] = useMutation<CreateUserResponse>(createUserQuery)
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
    onSubmit: async (values, { setSubmitting, setFieldError }) => {
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
          alert(result.apiToken)
          break
        case 'AlreadyLoggedIn':
          alert('You\'re already logged in')
          break
        case 'ValidationError':
          result.errors.forEach(({ field, errors: [ error ] }) => {
            setFieldError(field, error)
          })
          break
        }
      }

      setSubmitting(false)
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

      <Button
        disabled={formik.isSubmitting || !formik.isValid}
        sx={{ mt: 2 }}
        type="submit"
        variant="contained"
        fullWidth
      >
        Create Account
      </Button>
    </form>
  )
}
