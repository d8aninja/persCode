# Install with: pip install pyfmodex
# You'll also need FMOD Engine from http://www.fmod.org/download

import os
import sys

# SET THE PATH TO YOUR fmod.dll BEFORE importing pyfmodex
# Option 1: Specify full path to fmod.dll
fmod_path = r"C:\Program Files (x86)\FMOD SoundSystem\FMOD Studio API Windows\api\core\lib\x64\fmod.dll"

# Option 2: If fmod.dll is in the same directory as your script, use this instead:
# fmod_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "fmod.dll")

# Set the environment variable
os.environ["PYFMODEX_DLL_PATH"] = fmod_path

# NOW import pyfmodex
import pyfmodex
import time

# Initialize FMOD
system = pyfmodex.System()
system.init()

# Example 1: Basic audio playback
def basic_playback():
    # Load and play a sound
    sound = system.create_sound("path/to/your/audio.mp3")
    channel = sound.play()
    
    # Wait for sound to finish
    while channel.is_playing:
        system.update()
        time.sleep(0.01)
    
    sound.release()

# Example 2: Volume control
def volume_control():
    sound = system.create_sound("path/to/your/audio.wav")
    channel = sound.play()
    
    # Set volume (0.0 to 1.0)
    channel.volume = 0.5
    
    while channel.is_playing:
        system.update()
        time.sleep(0.01)
    
    sound.release()

# Example 3: Looping audio
def looping_audio():
    sound = system.create_sound(
        "path/to/background_music.mp3",
        mode=pyfmodex.flags.MODE.LOOP_NORMAL
    )
    
    channel = sound.play()
    
    # Play for a bit then stop
    time.sleep(5)
    channel.stop()
    
    sound.release()

# Example 4: 3D spatial audio
def spatial_audio():
    # Create a 3D sound
    sound = system.create_sound(
        "path/to/audio.wav",
        mode=pyfmodex.flags.MODE.THREED
    )
    channel = sound.play()
    
    # Set 3D position (x, y, z)
    channel.set_3d_attributes(position=(5.0, 0.0, 0.0))
    
    # Set listener position (camera/player position)
    system.set_3d_listener_attributes(
        listener_id=0,
        position=(0.0, 0.0, 0.0),
        velocity=(0.0, 0.0, 0.0),
        forward=(0.0, 0.0, 1.0),
        up=(0.0, 1.0, 0.0)
    )
    
    while channel.is_playing:
        system.update()
        time.sleep(0.01)
    
    sound.release()

# Example 5: Generate a tone (no audio file needed!)
def play_tone(frequency=440, duration=1.0):
    """Generate and play a sine wave tone"""
    # Create oscillator DSP
    dsp = system.create_dsp_by_type(pyfmodex.enums.DSP_TYPE.OSCILLATOR)
    dsp.set_parameter_float(pyfmodex.enums.DSP_OSCILLATOR.RATE, frequency)
    
    # Add to master channel group
    master = system.master_channel_group
    connection = master.add_dsp(0, dsp)
    
    # Play for specified duration
    time.sleep(duration)
    
    # Cleanup - remove DSP from the channel group
    master.remove_dsp(dsp)
    dsp.release()

# Example 6: Play a musical scale (no audio files needed!)
def play_musical_scale():
    """Play a C major scale using generated tones"""
    # Frequencies for C major scale (C, D, E, F, G, A, B, C)
    notes = [261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25, 587.33, 659.25, 698.46, 783.99, 880.00, 987.77, 1046.50]
    
    for freq in notes:
        print(f"Playing {freq} Hz")
        play_tone(freq, duration=0.5)
        time.sleep(0.1)  # Small pause between notes

# Run examples (uncomment the ones you want to try)
try:
    # If you don't have audio files, try these:
    print("Playing a 440 Hz tone (A note)...")
    play_tone(440, 1.0)
    
    print("\nPlaying musical scale...")
    play_musical_scale()
    
    # If you have audio files, uncomment these:
    # basic_playback()
    # volume_control()
    # looping_audio()
    # spatial_audio()
    
except Exception as e:
    print(f"Error: {e}")
finally:
    # Always cleanup
    system.release()