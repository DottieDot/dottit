import { Box, Button, TextField, Typography } from '@mui/material'

export default function SignupForm() {
  return (
    <Box >
      <Typography variant="h5">
        Signup
      </Typography>

      <TextField
        label="Email"
        margin="normal"
        type="email"
        fullWidth
      />

      <TextField
        label="Username"
        margin="normal"
        type="text"
        fullWidth
      />

      <TextField
        label="Password"
        margin="normal"
        type="password"
        fullWidth
      />

      <TextField
        label="Repeat Password"
        margin="normal"
        type="password"
        fullWidth
      />

      <Button sx={{ mt: 2 }} variant="contained" fullWidth>
        Create Account
      </Button>
    </Box>
  )
}
