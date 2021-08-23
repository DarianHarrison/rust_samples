// define a trait named Mlp with one method named draw:

pub trait Mlp {
    fn draw(&self);
    // tuple (source,dest,weight,boolean), if boolean is 1 source to destination is directed, else we just worry about edge connections
    // fn draw(&self, numLayers, inputDim, hiddenDim, outputDim);

}

        
// num_layers: number of layers in the neural networks (EXCLUDING the input layer). If num_layers=1, this reduces to linear model.
// input_dim: dimensionality of input features
// hidden_dim: dimensionality of hidden units at ALL layers
// output_dim: number of classes for prediction
// device: which device to use
        
