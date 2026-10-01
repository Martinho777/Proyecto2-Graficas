import struct
import sys
import zlib


def read_png(path):
    data = open(path, "rb").read()
    assert data[:8] == b"\x89PNG\r\n\x1a\n"
    pos = 8
    raw = b""
    while pos < len(data):
        size = struct.unpack(">I", data[pos:pos + 4])[0]
        kind = data[pos + 4:pos + 8]
        chunk = data[pos + 8:pos + 8 + size]
        pos += 12 + size
        if kind == b"IHDR":
            width, height, depth, color_type, _, _, interlace = struct.unpack(">IIBBBBB", chunk)
            assert (depth, color_type, interlace) == (8, 2, 0)
        elif kind == b"IDAT":
            raw += chunk
        elif kind == b"IEND":
            break
    decoded = zlib.decompress(raw)
    stride = width * 3
    rows = []
    previous = bytearray(stride)
    cursor = 0
    for _ in range(height):
        filter_type = decoded[cursor]
        cursor += 1
        current = bytearray(decoded[cursor:cursor + stride])
        cursor += stride
        for i in range(stride):
            left = current[i - 3] if i >= 3 else 0
            up = previous[i]
            up_left = previous[i - 3] if i >= 3 else 0
            if filter_type == 1:
                current[i] = (current[i] + left) & 255
            elif filter_type == 2:
                current[i] = (current[i] + up) & 255
            elif filter_type == 3:
                current[i] = (current[i] + ((left + up) // 2)) & 255
            elif filter_type == 4:
                prediction = left + up - up_left
                distances = (abs(prediction - left), abs(prediction - up), abs(prediction - up_left))
                current[i] = (current[i] + (left if distances[0] <= min(distances[1:]) else up if distances[1] <= distances[2] else up_left)) & 255
        rows.append(current)
        previous = current
    return width, height, rows


def write_ppm(source, output, target_width, target_height):
    width, height, rows = read_png(source)
    with open(output, "w", encoding="ascii") as file:
        file.write(f"P3\n{target_width} {target_height}\n255\n")
        for y in range(target_height):
            source_y = min(height - 1, y * height // target_height)
            row = rows[source_y]
            for x in range(target_width):
                source_x = min(width - 1, x * width // target_width)
                index = source_x * 3
                file.write(f"{row[index]} {row[index + 1]} {row[index + 2]} ")
            file.write("\n")


if __name__ == "__main__":
    write_ppm(sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]))
