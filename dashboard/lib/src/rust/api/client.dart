// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.33.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<APIClient>>
@sealed
class ApiClient extends RustOpaque {
  ApiClient.dcoDecode(List<dynamic> wire) : super.dcoDecode(wire, _kStaticData);

  ApiClient.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_ApiClient,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_ApiClient,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_ApiClientPtr,
  );

  Future<String> getBaseUrl({dynamic hint}) =>
      RustLib.instance.api.apiClientGetBaseUrl(that: this, hint: hint);

  Future<Response> loginUser(
          {required String email, required String password, dynamic hint}) =>
      RustLib.instance.api.apiClientLoginUser(
          that: this, email: email, password: password, hint: hint);

  // HINT: Make it `#[frb(sync)]` to let it become the default constructor of Dart class.
  static Future<ApiClient> newInstance(
          {required String baseUrl, dynamic hint}) =>
      RustLib.instance.api.apiClientNew(baseUrl: baseUrl, hint: hint);

  Future<Response> registerUser(
          {required String name,
          required String email,
          required String password,
          dynamic hint}) =>
      RustLib.instance.api.apiClientRegisterUser(
          that: this, name: name, email: email, password: password, hint: hint);
}

class Response {
  final int statusCode;
  final String body;

  const Response({
    required this.statusCode,
    required this.body,
  });

  @override
  int get hashCode => statusCode.hashCode ^ body.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Response &&
          runtimeType == other.runtimeType &&
          statusCode == other.statusCode &&
          body == other.body;
}