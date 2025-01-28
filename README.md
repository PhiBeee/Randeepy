# Randeepy (Placeholder name)

Simple website to randomly go through images in public chibisafe albums  
(This one is currently limited to albums hosted on eepy.ca)  
Currently uses one shared db file across everyone that uses the app instance (I'll see if I can figure out user specific or browser specific storage)  

# Functionality

- Save images into a favorites list to find them again later (and download later on)  
- Add/Remove your own custom public album to the list  
- Select which album you want images from

# Serving

You can run this locally by cloning the repo and then running the fololowing:  
```
dx serve
```  
in the main directory, which will serve the website to `localhost:8080` or basically `127.0.0.1:8080` in your browser.  

# Docker image
The Dockerfile is present in the repo so you can clone the repo and build the image yourself if you really want to.  
An already built image is also available [here](https://hub.docker.com/r/phibee/randeepy/tags)

# To-do's (at some point)
- Add ability to use other chibisafe instances  
- Make albums and favorites user bound instead of shared by every user of that instance of the app.
