from ultralytics import YOLO


def main():
	model = YOLO("runs/detect/train/weights/best.pt")

	model.predict(source="test_img.png", conf=0.25, show=True)

	input("press enter to close window")


if __name__ == "__main__":
	main()
