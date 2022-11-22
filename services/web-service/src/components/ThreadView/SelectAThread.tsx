import { Box, Typography } from '@mui/material'
import { Selection } from '../../undraw'

export default function SelectAThread() {
  return (
    <Box
      sx={{
        display:        'flex',
        height:         '100%',
        alignItems:     'center',
        justifyContent: 'center'
      }}
    >
      <Selection />

      <Typography variant="h3">
        Please select a thread.
      </Typography>
    </Box>
  )
}
