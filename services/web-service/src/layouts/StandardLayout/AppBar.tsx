import { AppBar as MuiAppBar, Button, CircularProgress, IconButton, Toolbar, Typography } from '@mui/material'
import { Link as RouterLink } from 'react-router-dom'
import { useUser } from '../../hooks'
import { AccountCircle as UserIcon, Logout as LogoutIcon } from '@mui/icons-material'
import { Fragment } from 'react'

function AccountButton() {
  const { state, setLoggedOut } = useUser()

  if (state.user) {
    return (
      <Fragment>
        <IconButton
          component={RouterLink}
          to={`/u/${state.user.username}`}
        >
          <UserIcon fontSize="inherit" />
        </IconButton>

        <IconButton onClick={setLoggedOut} sx={{ fontSize: '1.25em' }}>
          <LogoutIcon fontSize="inherit" />
        </IconButton>
      </Fragment>
    )
  }
  if (state.loggedIn) {
    return (
      <CircularProgress size="1.5em" />
    )
  }

  return (
    <Button
      color="inherit"
      component={RouterLink}
      to="/auth"
      variant="text"
    >
      Login
    </Button>
  )
}

export default function AppBar() {
  return (
    <MuiAppBar position="sticky">
      <Toolbar>
        <Typography
          component={RouterLink}
          sx={{
            flexGrow:       1,
            color:          'inherit',
            textDecoration: 'none'
          }}
          to="/"
          variant="h6"
          noWrap
        >
          Dottit
        </Typography>

        <AccountButton />
      </Toolbar>
    </MuiAppBar>
  )
}
