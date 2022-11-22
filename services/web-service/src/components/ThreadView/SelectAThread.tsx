import { Box, Typography } from '@mui/material'
import { Selection } from '../../undraw'
import { useTheme } from '@mui/material'

export default function SelectAThread() {
  const theme = useTheme()

  return (
    <Box
      sx={{
        display:        'flex',
        height:         '100%',
        alignItems:     'center',
        justifyContent: 'center',
        flexDirection:  'column'
      }}
    >
      <Box
        sx={{
          width:    500,
          maxWidth: '100%',
          mb:       2
        }}
      >
        <Selection
          primaryColor={theme.palette.primary.main}
        />
      </Box>

      <Typography variant="h3">
        Please select a thread.
      </Typography>
    </Box>
  )
}
