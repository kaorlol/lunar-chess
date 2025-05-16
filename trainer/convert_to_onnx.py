from ultralytics import YOLO

# Load your trained model
model = YOLO(
	r"c:\\Users\\root\\Code\\lunar-chess\\trainer\\runs\detect\\train\weights\best.pt"
)

# Export the model to ONNX format
model.export(format="onnx")
