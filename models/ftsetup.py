from transformers import AutoModelForCausalLM, AutoTokenizer
import torch
from peft import LoraConfig, get_peft_model # for peft and QLoRa later

tokenizer = AutoTokenizer.from_pretrained("meta-llama/Llama-2-7b-chat-hf") # load tokenizer
model_name = AutoModelForCausalLM.from_pretrained("meta-llama/Llama-2-7b-chat-hf") # load model

model = AutoModelForCausalLM.from_pretrained(
    model_name,
    load_in_4bit=True,  
    torch_dtype=torch.float16,
)

# test to see if model works
print("works") # TODO REMOVE

lora_config = LoraConfig( # basic config
    r = 16, 
    lora_alpha = 32,
    lora_dropout = 0.05,
    bias = "none",
    task_type = "CASUAL_LM"
)

peft_model = get_peft_model(model, lora_config) # paramater efficient 
peft_model.print_trainable_parameters() # print 