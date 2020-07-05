import sys

LINES_PER_FRAME = 14


def get_movie_frames(num_frames, input_file_path='sw1.txt'):
    movie = []  # store the movie as a list of frames
    with open(input_file_path) as input_movie_file:
        for i in range(num_frames):
            output_frame = read_frame(input_movie_file)
            movie.append(output_frame)
    return movie


def read_frame(movie_file):
    frame = ''
    for i in range(LINES_PER_FRAME):
        frame += movie_file.readline()
    return frame


def main():
    # default to reading first 10 frames if not given
    num_frames = int(sys.argv[1]) if len(sys.argv) > 1 else 10
    output_file_path = sys.argv[2] if len(sys.argv) > 2 else 'test_movie_1.txt'
    # movie start is not 0-indexed
    start_frame = int(sys.argv[3]) if len(sys.argv) > 3 else 1
    # movie end frame IS 0-indexed
    end_frame = start_frame + num_frames - 1
    movie = get_movie_frames(start_frame + num_frames)
    print(len(movie))

    split_movie = movie[start_frame - 1: end_frame]
    with open(output_file_path, 'w') as output_file:
        for frame in split_movie:
            output_file.write(frame)
    return


if __name__ == "__main__":
    main()
