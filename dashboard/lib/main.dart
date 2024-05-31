import 'package:flutter/material.dart';
import 'package:dashboard/src/rust/frb_generated.dart';
import 'package:dashboard/pages/register.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:awesome_notifications/awesome_notifications.dart';

Future<void> main() async {
  await dotenv.load(fileName: ".env");
  await AwesomeNotifications().initialize(
    'resource://drawable/flutter_logo',
    [
      NotificationChannel(
        channelKey: 'basic_channel',
        channelName: 'Basic notifications',
        channelDescription: 'Notification channel for basic tests',
        defaultColor: const Color(0xFF9D50DD),
        ledColor: const Color(0xFF9D50DD),
      )
    ],
    channelGroups: [
      NotificationChannelGroup(
        channelGroupKey: 'basic_channel',
        channelGroupName: 'Dashboard',
      )
    ],
  );

  bool isNotificationAllowed =
      await AwesomeNotifications().isNotificationAllowed();
  print('isNotificationAllowed: $isNotificationAllowed');
  if (!isNotificationAllowed) {
    await AwesomeNotifications().requestPermissionToSendNotifications();
  }
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  static var navigatorKey;

  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      home: RegisterPage(),
    );
  }
}
