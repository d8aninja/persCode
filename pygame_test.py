import pygame
import numpy as np

# Initialize pygame mixer
pygame.mixer.init(frequency=44100, size=-16, channels=2)  # 2 channels = stereo

# Generate a 440 Hz tone
sample_rate = 44100
duration = 1.0
frequency = 440

t = np.linspace(0, duration, int(sample_rate * duration))
wave = np.sin(2 * np.pi * frequency * t) * 32767
wave = wave.astype(np.int16)

# Convert to stereo (2D array with 2 columns for left and right channels)
stereo_wave = np.column_stack((wave, wave))

sound = pygame.sndarray.make_sound(stereo_wave)
sound.play()
pygame.time.wait(int(duration * 1000))

pygame.quit()