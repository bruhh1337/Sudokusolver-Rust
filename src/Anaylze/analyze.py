from pprint import pprint
import cv2
import numpy
import pathlib

sudoku = [
    [0, 0, 0,  0, 0, 0,  0, 0, 0],
    [0, 0, 0,  0, 0, 0,  0, 0, 0],
    [0, 0, 0,  0, 0, 0,  0, 0, 0],
    [0, 0, 0,  0, 0, 0,  0, 0, 0],
    [0, 0, 0,  0, 0, 0,  0, 0, 0],
    [0, 0, 0,  0, 0, 0,  0, 0, 0],
    [0, 0, 0,  0, 0, 0,  0, 0, 0],
    [0, 0, 0,  0, 0, 0,  0, 0, 0],
    [0, 0, 0,  0, 0, 0,  0, 0, 0],
]

working_dir = pathlib.Path(__file__).parent.resolve()

#bruh = cv2.imread(f"{working_dir}/src/Anaylze/Images/Capture.PNG")

#print(working_dir)

bruh = cv2.imread(f"{working_dir}/Images/Capture.PNG")

for i in range(1, 10):
    result = cv2.matchTemplate(bruh, cv2.imread(f"{working_dir}/Images/{i}.PNG"), cv2.TM_CCOEFF_NORMED)
    yloc, xloc = numpy.where(result >= 0.8)
    for (x, y) in zip(xloc, yloc):
        cv2.rectangle(bruh, (x, y), (x + 50, y + 50), (0, 0, 255), 2)
        col = x // (bruh.shape[0] // 9)
        row = y // (bruh.shape[1] // 9)
        sudoku[row][col] = i

cv2.imshow("Result", bruh)
cv2.waitKey()

for row in sudoku:
    print(" ".join(map(str, row)))