import { Box, Paper } from '@mui/material'
import { Outlet, useNavigate } from 'react-router'
import { useUser } from '../../hooks'

export default function LoginSignupPage() {
  const { state: { loggedIn }} = useUser()
  const navigate = useNavigate()

  if (loggedIn) {
    navigate('/')
  }

  return (
    <Box
      sx={{
        display:            'flex',
        width:              '100%',
        height:             '100%',
        alignItems:         'center',
        justifyContent:     'center',
        backgroundImage:    'url("/assets/background.jpg")',
        backgroundSize:     'cover',
        backgroundPosition: 'center'
      }}
    >
      <Paper
        sx={{
          width:    600,
          overflow: 'hidden',
          position: 'relative',
          p:        2
        }}
      >
        <Outlet />
      </Paper>
    </Box>
  )
}
