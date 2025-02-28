from transformers import AutoModelForCausalLM, AutoTokenizer
import torch
from peft import LoraConfig, get_peft_model # for peft and QLoRa later

model_name = "meta-llama/Llama-2-7b-chat-hf" #small scale
tokenizer = AutoTokenizer.from_pretrained(model_name)

model = AutoModelForCausalLM.from_pretrained(
    model_name,
    load_in_4bit=True,  
    torch_dtype=torch.float16,
)


# test to see if model works
print("works")
