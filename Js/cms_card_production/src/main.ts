// TODO: prerequisites - card product with min-max range
// 1. pan generator algorithm
// 2. store

let id_value = 1;

function sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const generateId = (): number => {
    const id = id_value;
    id_value += 1;
    return id;
};

type CardProduct = {
    id: number,
    name: string,
    bin: string,
    min_range: string,
    max_range: string,
    pan_length: number, // lets take as 16
    last_generated_pan: number // when created, this will be 0
}

const productStore: CardProduct[] = [];

type Card = {
    pan: string,
    seq_no: string,
    product_id: number,
    date_generated: Date,
}

const cardStore: Card[] = [];

function createCard(pan: string, cp: CardProduct): Card {
    return {
        pan,
        seq_no: "000",
        product_id: cp.id,
        date_generated: new Date(),
    }
}

type CardGenerationProgress = {
    product_id: number,
    progress: number,
    max: number
}

const createCardGenerationProgress = (cp: CardProduct, max: number): CardGenerationProgress => {
    return {
        product_id: cp.id,
        progress: 0,
        max
    };
}

function is_inbewteen_range(product: CardProduct, min: string, max: string): boolean {
    const product_min = Number(product.min_range);
    const product_max = Number(product.max_range);
    const this_min = Number(min);
    const this_max = Number(max);
    // a1 b1 a2 b2 -> positive || a1 a2 b1 b2 -> negative || a1 a2 b2 b1 -> negative
    return (product_max >= this_min && this_min >= product_min) ||
        (product_max >= this_max && this_max >= product_min);
}

const createProduct = (bin: string, min_range: string, max_range: string, product_name?: string): CardProduct => {
    // max range must be grater than min range
    if (Number(min_range) >= Number(max_range)) throw new Error("max range must be greater than the min range");

    // check if the range doesn't exist already for any other card product
    const is_match = productStore.some(product => product.bin === bin && is_inbewteen_range(product, min_range, max_range));
    if (is_match) {
        throw new Error(`card product with bin: ${bin} and range {${min_range}/${max_range}} already exists`);
    }
    const id = generateId();
    const new_product = {
        id,
        name: product_name ? product_name : `product_name_${id}`,
        bin,
        min_range,
        max_range,
        pan_length: 16,
        last_generated_pan: 0
    };
    productStore.push(new_product);
    return new_product;
};

class PanGenerator {
    // when we receive a request, mark the card production of this product.
    // If the request is received when card production is already in progress, decline new production
    progress_list: CardGenerationProgress[] = [];

    public async generateCard(card_product: CardProduct, no_of_cards: number) {
        // if no_of_cards is more than available among the list, decline
        const min_range = Number(card_product.min_range);
        const max_range = Number(card_product.max_range);

        const remaining_possible_cards: number = max_range - (card_product.last_generated_pan == 0 ? min_range : card_product.last_generated_pan);
        if (remaining_possible_cards < no_of_cards) {
            throw new Error(`maximum possible card generatable is: ${remaining_possible_cards}, your input of: ${no_of_cards} can't be genearated`);
        }

        // check if there is no other card production is in progress for the same card product
        const existingProgress = this.progress_list.find(i => i.product_id == card_product.id);
        if (existingProgress) {
            console.log(`Card production for the product is already in progress!`);
            console.log(`progress: ${existingProgress.progress}/${existingProgress.max}`);
            return;
        }

        // if all set, start producing card with gap, 
        // this gonna be an async function just so I can simulate the simultaneous pan generation
        let count = 0;
        const newProgress = createCardGenerationProgress(card_product, no_of_cards);
        this.progress_list.push(newProgress);
        console.log(`started for: ${JSON.stringify(newProgress, null, 2)}`);

        let new_last_count = card_product.last_generated_pan == 0 ? min_range : card_product.last_generated_pan;

        while (count < no_of_cards) {
            const len_of_last_count = new String(new_last_count + 1).length;
            const this_count = new_last_count + 1;
            const middle_part_len = 9 - len_of_last_count;
            const newPan = card_product.bin + "0".repeat(middle_part_len) + new String(this_count) + "0";
            count += 1;
            new_last_count += 1;
            const card = createCard(newPan, card_product);
            cardStore.push(card);

            // update progress on each iteration
            const updater = this.progress_list.find(i => i.product_id == card_product.id);
            if (updater) {
                updater.progress += 1;
            }

            // simulate load with an async sleep
            await sleep(200);
        }

        // after all cards are generated, update the last_pan in the product
        card_product.last_generated_pan = new_last_count;
        console.log(`generated card count: ${count}`);
        const removedProgressList = this.progress_list.filter(item => item.product_id !== card_product.id);
        this.progress_list = removedProgressList;
    }

    public getProgressList(): CardGenerationProgress[] {
        return this.progress_list;
    }
}

// -- usage code --

async function mainFn(): Promise<void> {
    const product1 = createProduct("123456", "000000000", "000002000");
    const product2 = createProduct("123456", "000002001", "000003999");
    const product3 = createProduct("123436", "000000000", "000020000");


    console.log(`all products: ${JSON.stringify(productStore, null, 2)}`);

    const panGen = new PanGenerator();
    const p1 = panGen.generateCard(product1, 2);
    const p2 = panGen.generateCard(product2, 3);

    await sleep(20);
    console.log(`progress: ${JSON.stringify(panGen.getProgressList(), null, 2)}`);

    await p1;
    await p2;
    // console.log(`All cards: ${JSON.stringify(cardStore, null, 2)}`); // 123456 000 019 999 0

    console.log(`progress list post card generation: ${JSON.stringify(panGen.getProgressList(), null, 2)}`);

}

mainFn().then(() => console.log("main FN done")).catch((err) => console.log(`err: ${err}`));