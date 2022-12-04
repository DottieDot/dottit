import { Outlet } from 'react-router'
import { Box } from '@mui/material'
import AppBar from './AppBar'

export default function StandardLayout() {
  return (
    <Box
      sx={{
        height:        '100%',
        display:       'flex',
        flexDirection: 'column'
      }}
    >
      <AppBar />
      <Outlet />
    </Box>
  )
}
