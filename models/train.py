import ftsetup
from transformers import Trainer, TrainingArguments

training_args = TrainingArguments(
    output_dir="~/blvflag/models/finetuned/",
    per_device_train_batch_size=4,
    gradient_accumulation_steps=8,
    warmup_steps=100,
    max_steps=5000,  
    learning_rate=2e-4,
    logging_dir="~/blvflag/trainlogs",
    save_strategy="epoch",
    push_to_hub=False
)

trainer = Trainer(
    model=ftsetup.peft_model,
    args=training_args,
    train_dataset="~/blvflag/models/mbpp.jsonl"
)

trainer.train() # train it 

