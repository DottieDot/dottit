import { alpha, Box, styled } from '@mui/material'

export default styled(Box)(({ theme }) => ({
  backdropFilter:  'blur(40px)',
  backgroundColor: alpha(theme.palette.getContrastText(theme.palette.background.paper), .05),
  color:           theme.palette.getContrastText(theme.palette.background.paper)
}))
