import { Box, Button, Paper } from '@mui/material'
import LoginForm from './LoginForm'
import SignupForm from './SignupForm'
import { useCallback, useEffect, useState } from 'react'
import { Swiper, SwiperSlide } from 'swiper/react'
import { West as LeftIcon, East as RightIcon } from '@mui/icons-material'
import './swiper.css'
import { Swiper as SwiperClass } from 'swiper/types'

export default function LoginSignupPage() {
  const [ slide, setSlide ] = useState(0)
  const [ swiper, setSwiper ] = useState<SwiperClass|null>(null)

  useEffect(() => {
    // Swiper is buggy as hell
    setTimeout(() => {
      swiper?.slideTo(slide)
    }, 0)
  }, [ swiper, slide ])

  const handleChangeSlide = useCallback(() => {
    setSlide(slide => (slide + 1) % 2)
  }, [ setSlide ])

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
        <Swiper
          draggable={false}
          onSwiper={setSwiper}
          spaceBetween={20}
          autoHeight
          noSwiping
        >
          <SwiperSlide>
            <LoginForm />
          </SwiperSlide>

          <SwiperSlide>
            <SignupForm />
          </SwiperSlide>
        </Swiper>

        <Box
          sx={{
            display:        'flex',
            justifyContent: 'flex-end',
            mt:             2
          }}
        >
          <Button
            color="secondary"
            endIcon={slide === 0 ? <RightIcon /> : null}
            onClick={handleChangeSlide}
            startIcon={slide === 1 ? <LeftIcon /> : null}
          >
            {slide === 0 ? (
              'Signup Instead'
            ) : (
              'Login Instead'
            )}
          </Button>
        </Box>
      </Paper>
    </Box>
  )
}
