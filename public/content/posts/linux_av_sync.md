---
title: "Linux A/V sync"
date: 2022-03-05T10:11:15+01:00
draft: false
---

In my opinion having a good video conference setup is essential in a remote job. The consensus on the net is that webcams are terrible as ... webcams. Instead, you get better quality using a compact camera with a capture card for the video and a separate microphone for audio. I use a Sony α5000 with an Audio-Technica at2020. Using separate devices for audio and video has as disadvantage they are no longer in sync. 

### Video setup
Using [OBS](https://obsproject.com) you can easily expose the capture card as a virtual camera (you will also need the package `v4l2loopback`). Unfortunately the virtual camera devices provides no audio. On Mac and Windows there is the [audio monitor](https://github.com/exeldro/obs-audio-monitor) plugin to get audio out of _OBS_ allowing us to sync the audio in _OBS_. On Linux there is no way to get synchronized audio out of obs.

Sometimes obs will not start the virtual camera in a sandboxed environment (flatpak/snap etc). You can manually load the needed kernel module: `modprobe v4l2loopback exclusive_caps=1 card_label='OBS Virtual Camera'`

For some reason my camera's white balance is off when using it as webcam. We can solve this using vl2-ctl (`sudo apt install v4l-utils`). It depends on your capture card/camera what you can change, a tweak to hue and saturation made a huge difference for me (I am no longer slightly green!).

### Audio sync
For Linux there is [EasyEffects](https://github.com/wwmm/easyeffects) to use it you will need to switch from _PulseAudio_ to the newer _PipeWire_ which is set to fully replace _PulseAudio_ in the future. For Ubuntu a guide can be found [guide](https://gist.github.com/the-spyke/2de98b22ff4f978ebf0650c90e82027e). Then install _EasyEffects_, I use the [flatpak](https://flathub.org/apps/details/com.github.wwmm.easyeffects) as it comes with all audio plugins. I feel more comfortable if _OBS_ can not claim the microphone therefore I disable all audio inputs in _OBS_ (file -> settings -> audio)

In _EasyEffects_
 - Go to the `Pipewire` tab at the top of the screen. 
 	- Under input devices ensure your microphone is picked up as default of select your microphone manually
	- Under Output Devices ensure default is not selected and pick a device that can not output sound!
	On my machine EasyEffects outputs whatever comes in over the microphone to the output device leading to a **feedback loop** that gets **painfully loud**. Picking a device that does not output audio, such as an unplugged headphone port, ensures this won't hurt you.
 - Go to the `Input` tab at the top of the screen. Now click the tab `plugins`
 - left top column `Add Plugins` add the `delay` plugin
 - toggle the button in the lower right corner to enable processing
 - use the spectrogram at the top to verify the delay is working (clap and see a delayed response in the spectrogram)
 - To use the now delayed sound choose the `EasyEffects Source` in your VoIP program.
