import CoreGraphics
import Foundation
import ImageIO
import UniformTypeIdentifiers

guard CommandLine.arguments.count == 3 else {
    fputs("Usage: flatten-icon.swift <input.png> <output.png>\n", stderr)
    exit(2)
}

let input = URL(fileURLWithPath: CommandLine.arguments[1]) as CFURL
let output = URL(fileURLWithPath: CommandLine.arguments[2]) as CFURL
let source = CGImageSourceCreateWithURL(input, nil)!
let image = CGImageSourceCreateImageAtIndex(source, 0, nil)!
let width = 1024
let height = 1024
let colorSpace = CGColorSpaceCreateDeviceRGB()
let context = CGContext(
    data: nil,
    width: width,
    height: height,
    bitsPerComponent: 8,
    bytesPerRow: 0,
    space: colorSpace,
    bitmapInfo: CGImageAlphaInfo.noneSkipLast.rawValue
)!

context.setFillColor(CGColor(red: 1, green: 1, blue: 1, alpha: 1))
context.fill(CGRect(x: 0, y: 0, width: width, height: height))
context.draw(image, in: CGRect(x: 0, y: 0, width: width, height: height))

let flattened = context.makeImage()!
let destination = CGImageDestinationCreateWithURL(output, UTType.png.identifier as CFString, 1, nil)!
CGImageDestinationAddImage(destination, flattened, nil)
precondition(CGImageDestinationFinalize(destination))
