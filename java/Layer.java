package java;
import java.util.Scanner;

public class Layer {
    public static final double[][] WEIGHTS = new double[][]{
        new double[]{0.689403  ,  0.88619095,  1.38559222, -0.54222447,  1.11872506},
        new double[]{-0.34322014, -0.11967749, -0.39488056,  1.61454952, -0.14985445},
        new double[]{1.01881731,  1.48689759,  0.72014976, -1.16128731,  1.14902306},
        new double[]{0.89593464,  0.54941881,  1.18199813,  0.60278761,  0.56533217},
        new double[]{0.4700622 , -0.53015655,  0.45113343, -0.16422655, -0.5543862},
    };

    public static final double[] BIASES = new double[]{0.21907845, -0.19085403,  0.08178166,  0.82804072, -0.10045063};

    public static double relu(double x) {
        if (x < 0) {
            return 0;
        } else {
            return x;
        }
    }

    public static double[] forward(double[] vec) {

        double o;
        double[] out = new double[5];
        for (int j = 0; j < BIASES.length; j++) {
            o = 0;
            for (int i = 0; i < WEIGHTS.length; i++) {
                o += WEIGHTS[i][j] * vec[i];
            }
            o += BIASES[j];
            o = relu(o);
            out[j] = o;
        }
        return out;
    }

    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);
        double[] vec = new double[5];
        for (int i = 0; i < 5; i++) {
            vec[i] = scan.nextDouble();
        }

        double[] out = Layer.forward(vec);
        String outp = "";
        for (int i = 0; i < 5; i++) {
            outp += out[i];
            if (i != 4) {
                outp += " ";
            }
        }
        System.out.println(outp);

    }
}