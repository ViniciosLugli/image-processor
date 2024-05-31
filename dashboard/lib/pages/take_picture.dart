import 'dart:convert';
import 'package:awesome_notifications/awesome_notifications.dart';
import 'package:dashboard/controller/notification_controller.dart';
import 'package:flutter/material.dart';
import 'package:image_picker/image_picker.dart';
import 'package:dashboard/client/singleton.dart';

class TakePicturePage extends StatefulWidget {
  const TakePicturePage({super.key});

  @override
  _TakePicturePageState createState() => _TakePicturePageState();
}

class _TakePicturePageState extends State<TakePicturePage> {
  Image? _image;
  bool _isLoading = false;
  final ImagePicker _picker = ImagePicker();
  var apiClient = Singleton.instance;

  @override
  void initState() {
    AwesomeNotifications().setListeners(
        onActionReceivedMethod: NotificationController.onActionReceivedMethod);
    super.initState();
  }

  Future<void> processImage(imagePath) async {
    setState(() {
      _isLoading = true;
    });

    print('Processing image: $imagePath');
    final response =
        await ((await apiClient).processImage(imagePath: imagePath));
    print('Response status code: ${response.statusCode}');
    final responseJson = jsonDecode(response.body);
    var imageBase64 = responseJson['image'];
    final commaIndex = imageBase64.indexOf(',');
    if (commaIndex != -1) {
      imageBase64 = imageBase64.substring(commaIndex + 1);
    }

    setState(() {
      _isLoading = false;
      _image = Image.memory(base64Decode(imageBase64));
    });

    AwesomeNotifications().createNotification(
      content: NotificationContent(
        id: 10,
        channelKey: 'basic_channel',
        title: 'Image processed!',
        body: 'The image has been processed successfully.',
      ),
    );
  }

  Future<void> _takePicture() async {
    final pickedFile = await _picker.pickImage(source: ImageSource.camera);

    setState(() {
      if (pickedFile != null) {
        processImage(pickedFile.path);
      } else {
        print('No image selected.');
      }
    });
  }

  Future<void> _pickFromGallery() async {
    final pickedFile = await _picker.pickImage(source: ImageSource.gallery);

    setState(() {
      if (pickedFile != null) {
        processImage(pickedFile.path);
      } else {
        print('No image selected.');
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Take Picture to grayscale!'),
      ),
      body: Center(
        child: _isLoading
            ? const CircularProgressIndicator()
            : Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: <Widget>[
                  _image == null ? const Text('No image selected.') : _image!,
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
