import { Box, Button, TextField, Typography } from '@mui/material'

export default function LoginForm() {
  return (
    <Box >
      <Typography variant="h5">
        Login
      </Typography>

      <TextField
        label="Email"
        margin="normal"
        type="email"
        fullWidth
      />

      <TextField
        label="Password"
        margin="normal"
        type="password"
        fullWidth
      />

      <Button sx={{ mt: 2 }} variant="contained" fullWidth>
        Login
      </Button>
    </Box>
  )
}
