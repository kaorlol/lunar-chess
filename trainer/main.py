import os

import yaml
from dotenv import load_dotenv
from roboflow import Roboflow
from ultralytics import YOLO

load_dotenv()


def main():
	rf = Roboflow(api_key=os.getenv("API_KEY"))
	project = rf.workspace(os.getenv("WORKSPACE_ID")).project(os.getenv("PROJECT_ID"))
	version = project.versions()[0]
	print(f"Version ID: {version.version}")

	dataset = version.download("yolov8")

	data_yaml_path = os.path.join(dataset.location, "data.yaml")

	with open(data_yaml_path, "r") as f:
		data_yaml = yaml.safe_load(f)
		print("Contents of data.yaml:")
		print(data_yaml)

	model = YOLO("yolo11n.pt")

	results = model.train(data=data_yaml_path, epochs=150, batch=32, device=0)

	print("Training results:")
	for result in results:
		print(result)

	os.rmdir(dataset.location)


if __name__ == "__main__":
	main()
