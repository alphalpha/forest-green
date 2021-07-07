# Forest Green
`Forest Green` is specifically written for using it with [phenological time lapse images and data from Monimet EU Life+ project](https://zenodo.org/communities/phenology_camera/?page=1&size=20).
It calculates the mean green value for a given area of each image in a given folder, generates an image of this color with the same dimension as the input image, and stitches both images horizontally together.  
Green image on the left, original image on the right. Adds annotation to the green image based on the `[<Camera_ID>]` found in this [Datasheet](https://zenodo.org/record/1066862) and the file name.  
  
usage:  
```forest-green [<configuration_file>]```  
example:  
```forest-green Configuration.toml```
