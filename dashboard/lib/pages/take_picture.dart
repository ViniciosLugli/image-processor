import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'dart:io';

class TakePicturePage extends StatefulWidget {
  const TakePicturePage({super.key});

  @override
  _TakePicturePageState createState() => _TakePicturePageState();
}

class _TakePicturePageState extends State<TakePicturePage> {
  File? _image;
  final ImagePicker _picker = ImagePicker();

  Future<void> _takePicture() async {
    final pickedFile = await _picker.pickImage(source: ImageSource.camera);

    setState(() {
      if (pickedFile != null) {
        _image = File(pickedFile.path);
      } else {
        print('No image selected.');
      }
    });
  }

  Future<void> _pickFromGallery() async {
    final pickedFile = await _picker.pickImage(source: ImageSource.gallery);

    setState(() {
      if (pickedFile != null) {
        _image = File(pickedFile.path);
      } else {
        print('No image selected.');
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Take Picture'),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            _image == null
                ? const Text('No image selected.')
                : Image.file(_image!),
            const SizedBox(height: 20.0),
            ElevatedButton(
              onPressed: _takePicture,
              child: const Text('Take Picture'),
            ),
            const SizedBox(height: 20.0),
            ElevatedButton(
              onPressed: _pickFromGallery,
              child: const Text('Pick from Gallery'),
            ),
          ],
        ),
      ),
    );
  }
}
